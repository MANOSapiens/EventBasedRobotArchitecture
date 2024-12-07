![robot](https://github.com/user-attachments/assets/4befe2dd-5cb4-4692-980e-9b636e3b4535)

# 🦎EventBasedRobotArchitecture

EventBasedRobotArchitecture is a custom backend for LEGO EV3 robots based on the ev3dev operating system. Designed by Chenpan Li from the **MANOSapiens Team** at the Martin-Andersen-Nexö Gymnasium Dresden, this innovative firmware introduces an event-driven architecture that enables highly efficient and precise task execution for robotic systems.

## Key Features

- **Event Loop-Based Architecture**  
  The system operates on an event loop where each task is defined by:
  - **Spawn Condition**: Specifies when the task should begin.
  - **Action Part**: Executes the core functionality of the task.
  - **Termination Condition**: Determines when the task concludes.

- **High-Performance Loop Rate**  
  Achieves an impressive **600 Hz loop rate**, allowing for rapid and precise execution of even the most complex sequences.

- **Compiler Integration**  
  A separate **Compiler** repository converts user-defined round sequences into structured tasks. These tasks are uploaded to the EV3 brick, streamlining the programming workflow.

- **Flexibility for Complex Sequences**  
  The architecture supports intricate execution flows, saving significant time and enhancing precision during robot operation.

---

## Achievements

Using EventBasedRobotArchitecture, the **MANOSapiens Team** reached the **FLL DACH-Finale 2024** in Davos, Switzerland. This firmware played a pivotal role in their success by enabling efficient and precise robot performance during competition rounds.

---

## Usage

This repository is provided **as-is** and has been archived. While no further changes will be made here, you are welcome to:
- Fork the repository to adapt it to your needs.
- Use it for your robotic systems, **with proper citation** of the original authors.

---

## Citation

If you use EventBasedRobotArchitecture in your projects or research, please include the following citation:
```
  Li, Chenpan. "EventBasedRobotArchitecture: A Custom FLL LEGO EV3 Backend." MANOSapiens Team, Martin-Andersen-Nexö Gymnasium Dresden, 2024.
```

---

## Notes

- **Archived Repository**  
  No further changes will be made to this repository. All modifications should be carried out on forks.

- **Open for Inspiration**  
  We encourage teams and developers to explore and build upon this architecture to push the boundaries of robotics!

---

## License

This project is distributed under an open-use license with citation required. See the [LICENSE](LICENSE) file for details.

---

Thank you for your interest in EventBasedRobotArchitecture. Best of luck with your robotic systems! 🚀
