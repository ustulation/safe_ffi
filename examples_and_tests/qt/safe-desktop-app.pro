#-------------------------------------------------
#
# Project created by QtCreator 2015-08-13T15:17:50
#
#-------------------------------------------------

QT       += core gui

greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

TARGET = safe-desktop-app
TEMPLATE = app


SOURCES += main.cc\
        mainwindow.cc

HEADERS  += mainwindow.h

LIBS += -L../ -lc_wrapper
LIBS += -L../../../../rust/target/release -lsafe_ffi
LIBS += -lsodium -ldl

FORMS    += mainwindow.ui
