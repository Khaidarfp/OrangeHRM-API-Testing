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

response = WS.sendRequestAndVerify(findTestObject('Leave/GET_Employee_Leave_Requests'))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'data[0].employeeName', 'Kijang Satu')

WS.verifyElementPropertyValue(response, 'data[0].employeeId', '1')

WS.verifyElementPropertyValue(response, 'data[0].id', '33')

WS.verifyElementPropertyValue(response, 'data[0].fromDate', '2022-09-30')

WS.verifyElementPropertyValue(response, 'data[0].toDate', '2022-09-30')

WS.verifyElementPropertyValue(response, 'data[0].type', 'Cuti')

WS.verifyElementPropertyValue(response, 'data[0].leaveBalance', '27.00')

WS.verifyElementPropertyValue(response, 'data[0].numberOfDays', '1.00')

WS.verifyElementPropertyValue(response, 'data[0].comments[0].user', 'API User')

WS.verifyElementPropertyValue(response, 'data[0].comments[0].date', '2020-10-23')

WS.verifyElementPropertyValue(response, 'data[0].comments[0].time', '04:03:43')

WS.verifyElementPropertyValue(response, 'data[0].comments[0].comment', 'test')

WS.verifyElementPropertyValue(response, 'data[0].days[0].date', '2022-09-30')

WS.verifyElementPropertyValue(response, 'data[0].days[0].status', 'PENDING APPROVAL')

WS.verifyElementPropertyValue(response, 'data[0].days[0].duration', '8.00')

WS.verifyElementPropertyValue(response, 'data[0].days[0].durationString', '')

