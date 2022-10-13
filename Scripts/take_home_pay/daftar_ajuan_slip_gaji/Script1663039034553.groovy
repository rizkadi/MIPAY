import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.click(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/span_Daftar Ajuan Slip Gaji'))

WebUI.setText(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input_Daftar Ajuan Slip Gaji_keyword'), 
    '')

WebUI.click(findTestObject('take_home_pay/daftar_ajuan_slip_gaji/a_Form'))

WebUI.switchToWindowIndex(1)

WebUI.setText(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input__gapok'), '1')

WebUI.setText(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input__tunj_suami_istri'), '1')

WebUI.setText(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input__tunj_anak'), '1')

WebUI.setText(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input__tunj_fungsional'), '1')

WebUI.setText(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input__tunj_perbaikan'), '1')

WebUI.setText(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input__tunj_lain'), '1')

WebUI.setText(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input__tunj_umum'), '1')

WebUI.setText(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input__tunj_papua'), '1')

WebUI.setText(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input__tunj_daerah'), '1')

WebUI.setText(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input__tunj_beras'), '1')

WebUI.setText(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input__tunj_pajak'), '1')

WebUI.setText(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input__tunj_pembulatan'), '1')

WebUI.click(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input_Tampilkan_cek_iwp'))

WebUI.click(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input_Tampilkan_cek_pot_beras'))

WebUI.click(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input_Tampilkan_cek_sewarmh'))

WebUI.click(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input_Tampilkan_cek_utang_lebih'))

WebUI.click(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input_Tampilkan_cek_pot_bpjs'))

WebUI.click(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input_Tampilkan_cek_taperum'))

WebUI.click(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input_Tampilkan_cek_pot_lain'))

WebUI.click(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input_Tampilkan_cek_pph'))

WebUI.click(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input_Tampilkan_cek_tunj_serdos'))

WebUI.click(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input_Tampilkan_cek_tunj_guru_besar'))

WebUI.click(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input_Tampilkan_cek_tpb'))

WebUI.click(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input_Tampilkan_cek_ikits'))

WebUI.click(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/input_Tampilkan_cek_uang_makan'))

WebUI.click(findTestObject('Object Repository/take_home_pay/daftar_ajuan_slip_gaji/button_Generate'))

