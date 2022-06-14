<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Air LFS</name>
   <tag></tag>
   <elementGuidId>810ee6b8-28ea-4595-ab5c-e9dac04ae4d8</elementGuidId>
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
		&lt;air:LowFareSearchReq AuthorizedBy=&quot;SUVO&quot; TargetBranch=&quot;${TargetBranch}&quot; SolutionResult=&quot;true&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
			&lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot;/>
			&lt;air:SearchAirLeg>
				&lt;air:SearchOrigin>
					&lt;com:Airport Code=&quot;SYD&quot;/>
				&lt;/air:SearchOrigin>
				&lt;air:SearchDestination>
					&lt;com:Airport Code=&quot;MEL&quot;/>
				&lt;/air:SearchDestination>
				&lt;air:SearchDepTime PreferredTime=&quot;2021-12-20&quot;/>
			&lt;/air:SearchAirLeg>
			&lt;air:AirSearchModifiers>
				&lt;air:PreferredProviders>
					&lt;com:Provider Code=&quot;${Provider}&quot;/>
				&lt;/air:PreferredProviders>
				&lt;air:PermittedCarriers>
					&lt;com:Carrier Code=&quot;QF&quot;/>
				&lt;/air:PermittedCarriers>
			&lt;/air:AirSearchModifiers>
			&lt;com:SearchPassenger Code=&quot;ADT&quot;/>
		&lt;/air:LowFareSearchReq>
	&lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://WAS-UNIVERSALAPI-33890-100.UT.TVLPORT.COM:33890/kestrel/AirService</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.TargetBranch</defaultValue>
      <description></description>
      <id>f9c7205c-e6e1-40d3-9501-460c8482d463</id>
      <masked>false</masked>
      <name>TargetBranch</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ProviderID</defaultValue>
      <description></description>
      <id>a5f242d1-3ff4-42dd-a548-cc8646b48550</id>
      <masked>false</masked>
      <name>Provider</name>
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
