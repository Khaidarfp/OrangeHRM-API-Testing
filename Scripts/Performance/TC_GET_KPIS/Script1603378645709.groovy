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
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

response = WS.sendRequestAndVerify(findTestObject('Performance/GET_KPIS'))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'data[0].id', "1")

WS.verifyElementPropertyValue(response, 'data[0].jobTitle.id', "1")

WS.verifyElementPropertyValue(response, 'data[0].jobTitle.jobTitleName', "QA Engineer")

WS.verifyElementPropertyValue(response, 'data[0].jobTitle.jobDescription', "Staff")

WS.verifyElementPropertyValue(response, 'data[0].jobTitle.note', "")

WS.verifyElementPropertyValue(response, 'data[0].jobTitle.isDeleted', "0")

WS.verifyElementPropertyValue(response, 'data[0].kpi', "25")

WS.verifyElementPropertyValue(response, 'data[0].minRating', "20")

WS.verifyElementPropertyValue(response, 'data[0].maxRating', "30")