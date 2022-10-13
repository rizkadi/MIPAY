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

WebUI.click(findTestObject('Object Repository/take_home_pay/bukti_pot_pajak/span_Bukti Potong Pajak'))

WebUI.verifyTextPresent('Cetak Bukti Potong Pajak', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/take_home_pay/bukti_pot_pajak/select_- Pilih Jenis -                     _86c328'), 
    'bukti', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/take_home_pay/bukti_pot_pajak/select_- Pilih Tahun -                     _971461'), 
    '2022', true)

WebUI.click(findTestObject('Object Repository/take_home_pay/bukti_pot_pajak/button_Cetak'))

WebUI.switchToWindowTitle('Bukti Pemotongan Pajak')

WebUI.verifyTextPresent('BUKTI PEMOTONGAN PAJAK PENGHASILAN', true)

WebUI.switchToWindowTitle('Take Home Pay • myITS Payroll')

WebUI.selectOptionByValue(findTestObject('Object Repository/take_home_pay/bukti_pot_pajak/select_- Pilih Jenis -                     _86c328'), 
    'rekap', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/take_home_pay/bukti_pot_pajak/select_- Pilih Tahun -                     _971461'), 
    '2022', true)

WebUI.click(findTestObject('Object Repository/take_home_pay/bukti_pot_pajak/button_Cetak'))

WebUI.switchToWindowTitle('Rekap Bukti Pemotongan Pajak')

WebUI.verifyTextPresent('REKAP BUKTI PEMOTONGAN PPh 21 Tahun 2022', true)

