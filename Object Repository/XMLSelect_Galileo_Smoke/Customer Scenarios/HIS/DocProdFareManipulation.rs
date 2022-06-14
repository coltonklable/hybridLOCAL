<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DocProdFareManipulation</name>
   <tag></tag>
   <elementGuidId>36f2df25-6a61-4007-95a5-9c1077d39f9a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic R1dTX1VBVF9QNzExNDAyNDpFY2Q4NCRJSXAq</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:web=&quot;http://webservices.galileo.com&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;web:SubmitXmlOnSession>
         &lt;web:Token>${TokenNumber}&lt;/web:Token>
         &lt;web:Request>
            &lt;DocProdFareManipulation_27>
               &lt;TicketingMods>
                  &lt;ElectronicTicketFailed>
                     &lt;CancelInd>Y&lt;/CancelInd>
                     &lt;IssuePaperTkInd/>
                     &lt;IssuePaperTkToSTP/>
                     &lt;STPlocation/>
                  &lt;/ElectronicTicketFailed>
                  &lt;FareNumInfo>
                     &lt;FareNumAry>
                        &lt;FareNum>1&lt;/FareNum>
                     &lt;/FareNumAry>
                  &lt;/FareNumInfo>
                  &lt;TicketingControl>
                     &lt;TransType>TK&lt;/TransType>
                  &lt;/TicketingControl>
               &lt;/TicketingMods>
            &lt;/DocProdFareManipulation_27>
         &lt;/web:Request>
         &lt;web:Filter>
            &lt;_/>
         &lt;/web:Filter>
      &lt;/web:SubmitXmlOnSession>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${GlobalVariable.Endpoint}</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.TokenNumber</defaultValue>
      <description></description>
      <id>a08c6f20-efff-4834-8b43-d49d41c68c6c</id>
      <masked>false</masked>
      <name>TokenNumber</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

println(&quot;RecordLocator : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.DocProdFareManipulation_27.Ticketing.RecordLocator.RecLoc')))

println(&quot;TextMessages : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.DocProdFareManipulation_27.Ticketing.TextMsg.Txt')))


assertThat(response.getResponseText()).contains('ITIN/INVOICE GENERATED')
assertThat(response.getResponseText()).contains('ELECTRONIC TKT GENERATED')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
