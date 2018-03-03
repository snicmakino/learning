<?php

for ($i = 0; $i < 5; $i++) {
    $pid = pcntl_fork();
    if ($pid == 0) {
        sleep(5);
        exit;
    }
}
