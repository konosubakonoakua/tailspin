package handler

import (
	"os"
	"os/exec"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/nxadm/tail"

	"spin/conf"
)

type editorFinishedMsg struct{ err error }

type Model struct {
	TailFile   *tail.Tail
	TempFile   *os.File
	Config     *conf.Config
	hasStarted bool
}

func openEditor(m *Model) tea.Cmd {
	c := less(m.TempFile.Name(), m.Config)

	return tea.ExecProcess(c, func(err error) tea.Msg {
		return editorFinishedMsg{err}
	})
}

func less(path string, config *conf.Config) *exec.Cmd {
	args := []string{
		"--RAW-CONTROL-CHARS",
		"--ignore-case",
	}

	if config.Follow {
		args = append(args, "+F")
	}

	args = append(args, path)

	command := exec.Command("less", args...)

	command.Stdin = os.Stdin
	command.Stdout = os.Stdout

	return command
}

func (m Model) Init() tea.Cmd {
	return nil
}

func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg.(type) {
	case editorFinishedMsg:
		return m, tea.Quit
	}

	if m.hasStarted {
		return m, nil
	}

	m.hasStarted = true
	return m, openEditor(&m)
}

func (m Model) View() string {
	return ""
}
