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

'wait 2 seconds before starting'
WebUI.delay(1)

'verify if page is loaded'
if (WebUI.verifyElementPresent(findTestObject('Orange HRM_Existe'), 0, FailureHandling.OPTIONAL)) {
    WebUI.click(findTestObject('RecruitementBtn'))

    WebUI.click(findTestObject('CandidateBtn'))

    WebUI.waitForPageLoad(1)

    not_run: WebUI.verifyElementPresent(findTestObject('Liste Candidate_Existe'), 2)

    WebUI.click(findTestObject('test/Page_OrangeHRM/i_add'))

    not_run: WebUI.verifyElementPresent(findTestObject('Add candidate Title_Existe'), 2)

    CustomKeywords.'com.hajar.UploadFilePDF.uploadFile'(findTestObject('Page_OrangeHRM/div_Select Resume'), 'C:\\certificateNICE_HajarHerrou.pdf')

    WebUI.setText(findTestObject('test2/Page_OrangeHRM/input_Accepts .docx, .doc, .odt, .pdf, .rtf_b38173'), FN)

    WebUI.setText(findTestObject('Page_OrangeHRM/input_Middle Name_addCandidatelastName'), MN)

    WebUI.setText(findTestObject('Page_OrangeHRM/input__addCandidateemail'), Email)

    WebUI.click(findTestObject('Page_OrangeHRM/div_--Select--'))

    WebUI.setText(findTestObject('test2/Page_OrangeHRM/input_--Select--_employee-search validate'), Post)

    WebUI.click(findTestObject('Page_OrangeHRM/p_UIUX Engineer'))

    WebUI.click(findTestObject('Page_OrangeHRM/a_Save'))
} else {
    WebUI.setAlertText('There is no connection to continue')
}

