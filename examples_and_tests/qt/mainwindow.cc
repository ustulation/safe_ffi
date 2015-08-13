#include "mainwindow.h"
#include "ui_mainwindow.h"

#include "../../c/c_wrapper.h"


MainWindow::MainWindow(QWidget *parent) :
  QMainWindow(parent),
  ui(new Ui::MainWindow),
  is_dns_registered(false)
{
  ui->setupUi(this);

  connect(ui->pb_sub_dir_create, SIGNAL(clicked()), SLOT(on_create_sub_dir()));
  connect(ui->pb_file_create, SIGNAL(clicked()), SLOT(on_create_file()));
  connect(ui->pb_dns, SIGNAL(clicked()), SLOT(on_register_dns_or_add_service()));
}

MainWindow::~MainWindow()
{
  delete ui;
}

void MainWindow::on_create_sub_dir() {
  std::string dir_path = ui->create_sub_dir->text().toStdString();
  ui->result->setText(QString("%1").arg(c_create_sub_directory(dir_path.c_str(), false)));
}

void MainWindow::on_create_file() {
  std::string file_path = ui->create_file->text().toStdString();
  std::string file_content = ui->file_content->toHtml().toStdString();
  ui->result->setText(QString("%1").arg(c_create_file(file_path.c_str(), (uint8_t*)file_content.c_str(), file_content.size())));
}

void MainWindow::on_register_dns_or_add_service() {
  std::string long_name = ui->long_name->text().toStdString();
  std::string service_name = ui->service_name->text().toStdString();
  std::string service_path = ui->service_path->text().toStdString();

  if (is_dns_registered) {
    ui->result->setText(QString("%1").arg(c_add_service(long_name.c_str(), service_name.c_str(), service_path.c_str())));
  } else {
    ui->result->setText(QString("%1").arg(c_register_dns(long_name.c_str(), service_name.c_str(), service_path.c_str())));
    if (ui->result->text() == "0") {
      is_dns_registered = true;
    }
  }
}
