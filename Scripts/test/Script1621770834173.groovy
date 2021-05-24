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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://hajarherrou-trials71.orangehrmlive.com/')

WebUI.click(findTestObject('Object Repository/Page_OrangeHRM/span_Username'))

WebUI.setText(findTestObject('Object Repository/Page_OrangeHRM/input_LOGIN Panel_txtUsername'), 'Admin')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_OrangeHRM/input_Username_txtPassword'), 'ULMqZvNddKgKpAgodwnFCA==')

WebUI.click(findTestObject('Object Repository/Page_OrangeHRM/input_Password_Submit'))

WebUI.click(findTestObject('Object Repository/test2/Page_Dashboard/span_Recruitment'))

WebUI.click(findTestObject('Object Repository/test2/Page_Dashboard/span_Candidates'))

WebUI.click(findTestObject('Object Repository/Page_OrangeHRM/i_add_1'))

not_run: WebUI.click(findTestObject('Page_OrangeHRM/div_Select Resume'))

not_run: WebUI.sendKeys(findTestObject(null), Keys.chord(Keys.ENTER, Keys.TAB, Keys.ENTER))

CustomKeywords.'com.hajar.UploadFilePDF.uploadFile'(findTestObject('Page_OrangeHRM/div_Select Resume'), 'C:\\certificateNICE_HajarHerrou.pdf')

not_run: WebUI.setText(findTestObject('Object Repository/Page_OrangeHRM/input_Accepts .docx, .doc, .odt, .pdf, .rtf_b38173'), 
    'hajar')

not_run: WebUI.click(findTestObject('Object Repository/Page_OrangeHRM/label_Last Name'))

not_run: WebUI.setText(findTestObject('Object Repository/Page_OrangeHRM/input_Middle Name_addCandidatelastName'), 'Her')

not_run: WebUI.setText(findTestObject('Object Repository/Page_OrangeHRM/input__addCandidateemail'), 'hh66@gmail.com')

not_run: WebUI.click(findTestObject('Object Repository/Page_OrangeHRM/div_--Select--'))

not_run: WebUI.setText(findTestObject('Object Repository/Page_OrangeHRM/input_--Select--_employee-search validate'), 'UX')

not_run: WebUI.click(findTestObject('Object Repository/Page_OrangeHRM/p_UIUX Engineer'))

not_run: WebUI.click(findTestObject('Object Repository/Page_OrangeHRM/a_Save'))

not_run: WebUI.switchToWindowTitle('')

not_run: WebUI.click(findTestObject('Object Repository/Page_OrangeHRM/td_hajar  Her'))

