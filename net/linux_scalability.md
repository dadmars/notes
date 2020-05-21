# 惊群

服务器为每个到来的连接建立一个线程

Linux 2.2.9 kernel (and earlier)

当多个线程对同一个 socket 调用 accept(), 这些线程进入到同一个等待队列, 等待一个新建立的连接将它们唤醒。当一个新的 TCP connection is accepted, wake_up_interruptible() 函数被调用，唤醒等待的线程。此函数遍历等待队列，唤醒所有线程。然后，其中的一个线程把其它线程又送回等待队列。此问题叫作惊群

socket 结构包含下面一些函数:

    sock->state_change.................... (pointer to sock_def_wakeup)
    sock->data_ready...................... (pointer to sock_def_readable)
    sock->write_space..................... (pointer to tcp_write_space)
    sock->error_report.................... (pointer to sock_def_error_report)

上面每个函数都会调用 wake_up_interruptible()。即第调用一次以上函数，线程都会被唤醒。accept() 中按序调用下面函数： tcp_write_space(), sock_def_readable(), sock_def_wakeup()

调用 wake_up_interruptible() 如此普遍, 其它地方也会产生惊群问题。

# Solutions

在线程进入等待队列时决定线程 should be exclusive or not shouldn't occur, 而不是在唤醒线程时。新增两函数 wake_one(), wake_one_interruptible() 

    sock->state_change (pointer to tcp_wakeup).............. wake_one_interruptible()
    sock->data_ready   (pointer to tcp_data_ready).......... wake_one_interruptible()
    sock->write_space  (pointer to tcp_write_space)......... wake_one_interruptible()
    sock->error_report (pointer to sock_def_error_report)... wake_up_interruptible()