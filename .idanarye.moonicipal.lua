local moonicipal = require'moonicipal'
local T = moonicipal.tasks_file()

local channelot = require'channelot'
local blunder = require'blunder'

T{alias=':2'}
function T:build_profile()
    local cc = self:cached_choice{key = vim.inspect}
    cc('dev')
    cc('release')
    return cc:select()
end

local function gen_all_implemented_days()
    local result = {}
    local pattern = vim.regex[=[\v^day\zs\d+\ze\.rs$]=]
    for name, typ in vim.fs.dir('src') do
        if typ == 'file' then
            local s, e = pattern:match_str(name)
            if s then
                table.insert(result, tonumber(name:sub(s + 1, e)))
            end
        end
    end
    table.sort(result)
    return result
end

function T:clean()
    vim.cmd'!cargo clean'
end

function T:check()
    blunder.run{'cargo', 'check', '-q', '--profile', T:build_profile()}
end

function T:build()
    blunder.run{'cargo', 'build', '-q', '--profile', T:build_profile()}
end

function T:run()
    channelot.windowed_terminal_job{'cargo', 'run', '--profile', T:build_profile(), '--', '--day', vim.fn.max(gen_all_implemented_days())}
end

function T:act()
    channelot.windowed_terminal_job{'cargo', 'run', '--profile', T:build_profile()}
end

T{alias = ':0'}
function T:add_day()
    local template_directories = {}
    for template in vim.fs.dir('.copier/') do
        if vim.startswith(template, 'day-template-') then
            table.insert(template_directories, template)
        end
    end
    local chosen_template = moonicipal.select(template_directories) or moonicipal.abort()
    local template_path = '.copier/' .. chosen_template
    moonicipal.fix_echo()
    local day = tonumber(moonicipal.input{
        default = vim.fn.strftime('%d'),
        prompt = 'Day number: ',
    }) or moonicipal.abort('No number selected')
    channelot.windowed_terminal_job{
        'copier', 'copy',
        '-fd', 'day=' .. day,
        template_path,
        '.',
    }:check()
    do
        local lines = vim.fn.readfile('src/lib.rs')
        table.insert(lines, ('pub mod day%s;'):format(day))
        vim.fn.writefile(lines, 'src/lib.rs')
    end

    do
        local lines = vim.fn.readfile('src/main.rs')
        if lines[#lines] ~= '}' then
            error('Malformed main.rs')
        end
        table.insert(lines, #lines, ('    day%s : generator => part_1, part_2;'):format(day))
        dump(lines)
        vim.fn.writefile(lines, 'src/main.rs')
    end
    vim.cmd.checktime()
end

T{alias=':3'}
function T:demonstration_input()
    return self:cached_data_cell{}
end

function T:go()
    local data = T:demonstration_input() or moonicipal.abort('No demonstration input')
    local j = channelot.windowed_terminal_job({
        RUST_BACKTRACE = 1,
    }, {'cargo', 'run', '--profile', T:build_profile(), '--', '--day', vim.fn.max(gen_all_implemented_days()), '--stdin'})
    j:write(data)
    j:write('\n\4')
    j:close_stdin()
    j:wait()
end

function T:run_cargo_fmt()
    vim.cmd'!cargo fmt'
end

function T:clippy()
    blunder.run { 'cargo', 'clippy', '-q' }
end

