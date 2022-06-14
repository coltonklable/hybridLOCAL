<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AirTicketing</name>
   <tag></tag>
   <elementGuidId>889f125b-9d9a-4221-a5db-66328314943c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dUFQSTg4NDczMTI1ODUtMmJjMjRlMjQ6TmV3cGFzc180VSoyMA==</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
   &lt;soapenv:Body>
      &lt;air:AirTicketingReq ReturnInfoOnFail=&quot;false&quot; TraceId=&quot;3a142551-c090-4793-a50c-00230ccd0f71&quot; TargetBranch=&quot;${TargetBranch}&quot; AuthorizedBy=&quot;uAPI&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;uAPI&quot;/>
&lt;air:AirReservationLocatorCode>${AirReservationRecordLocator}&lt;/air:AirReservationLocatorCode>
        
      &lt;/air:AirTicketingReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://WAS-UNIVERSALAPI-33890-100.UT.TVLPORT.COM:33890/kestrel/UniversalRecordService</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.ReservationLocatorCode</defaultValue>
      <description></description>
      <id>17981a2d-d242-4d0e-b32c-fef7962c68d2</id>
      <masked>false</masked>
      <name>AirReservationRecordLocator</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
