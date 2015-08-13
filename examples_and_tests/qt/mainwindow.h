#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>

namespace Ui {
class MainWindow;
}

class MainWindow : public QMainWindow
{
  Q_OBJECT

public:
  explicit MainWindow(QWidget *parent = 0);
  ~MainWindow();

private Q_SLOTS:
  void on_create_sub_dir();
  void on_create_file();
  void on_register_dns_or_add_service();

private:
  Ui::MainWindow *ui;
  bool is_dns_registered;
};

#endif // MAINWINDOW_H
