local moonicipal = require'moonicipal'
local T = moonicipal.tasks_file()

local channelot = require'channelot'

T{alias = ':1'}
function T:choose_day()
    return 1
end

T{alias = ':2'}
function T:choose_part()
    local cc = self:cached_choice {
        key = function(n)
            return n
        end,
        format = function(n)
            return ('Part %s'):format(n)
        end,
    }
    cc(1)
    cc(2)
    return cc:select()
end

T{alias = ':3'}
function T:example_input()
    return self:cached_data_cell {
    }
end

function T:target()
    local day = T:choose_day()
    local part = T:choose_part()
    return {
        day = day,
        part = part,
        script = ('nu-solutions/day%s.nu'):format(day),
        subcommand = ('part%s'):format(part),
        input = ('inputs/day%s.txt'):format(day),
    }
end

function T:run()
    channelot.windowed_terminal_job({
        input = T:target().input,
    }, {'nu', T:target().script, T:target().subcommand})
end

function T:go()
    local data = vim.fn.split(T:example_input(), '\n')
    vim.fn.writefile(data, '/tmp/example_input')
    channelot.windowed_terminal_job({
        input = '/tmp/example_input',
    }, {'nu', T:target().script, T:target().subcommand})
end

T{alias = ':0'}
function T:download_day()
    local target = T:target()
    channelot.windowed_terminal():with(function(t)
        t:job{'mkdir', '-vp', 'inputs'}:wait()
        t:job{
            'aoc',
            'download',
            '-d', target.day,
            '-I',
            '-i', target.input,
        }:wait()
    end)
end
