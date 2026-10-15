Unicode true
!include "MUI2.nsh"
Name "Vynamp"
Icon "${ICON}"
UninstallIcon "${ICON}"
OutFile "${OUTPUT}"
InstallDir "$LOCALAPPDATA\Programs\Vynamp"
RequestExecutionLevel user
VIProductVersion "${NUMERIC_VERSION}"
VIAddVersionKey "ProductName" "Vynamp"
VIAddVersionKey "FileVersion" "${VERSION}"
!insertmacro MUI_PAGE_LICENSE "${STAGING}\LICENSE"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "PortugueseBR"
Section "Vynamp"
  SetOutPath "$INSTDIR"
  File /r "${STAGING}\*"
  WriteUninstaller "$INSTDIR\Uninstall.exe"
  CreateShortcut "$SMPROGRAMS\Vynamp.lnk" "$INSTDIR\vynamp.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Vynamp" "DisplayName" "Vynamp"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Vynamp" "DisplayVersion" "${VERSION}"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Vynamp" "UninstallString" '"$INSTDIR\Uninstall.exe"'
SectionEnd
Section "Uninstall"
  Delete "$SMPROGRAMS\Vynamp.lnk"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Vynamp"
  RMDir /r "$INSTDIR"
  ; User library/settings/downloads live outside install directory and are preserved.
SectionEnd
