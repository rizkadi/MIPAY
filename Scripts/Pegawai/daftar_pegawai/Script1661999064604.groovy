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

WebUI.click(findTestObject('Object Repository/Pegawai/daftar_pegawai/span_Daftar Pegawai'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Pegawai/daftar_pegawai/select_102550Semua'), '25', true)

WebUI.verifyTextPresent('1 sampai 25', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Pegawai/daftar_pegawai/select_102550Semua'), '50', true)

WebUI.verifyTextPresent('1 sampai 50', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Pegawai/daftar_pegawai/select_102550Semua'), '-1', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Pegawai/daftar_pegawai/select_102550Semua'), '10', true)

WebUI.verifyTextPresent('1 sampai 10', true)

WebUI.setText(findTestObject('Object Repository/Pegawai/daftar_pegawai/input_Daftar Pegawai_keyword'), 'Rizky Adi ')

WebUI.verifyTextPresent('Rizky Adi Putra S.Kom', true)

WebUI.click(findTestObject('Object Repository/Pegawai/daftar_pegawai/a_1998202241014'))

WebUI.verifyTextPresent('3515172404980002', true)

WebUI.verifyTextPresent('Detail Pegawai', true)

