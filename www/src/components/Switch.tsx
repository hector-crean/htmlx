import React from 'react';

interface CaseProps {
    condition: boolean;
    children: React.ReactNode;
}

interface DefaultProps {
    children: React.ReactNode;
}

const Case: React.FC<CaseProps> = ({ children }) => <>{children}</>;
const Default: React.FC<DefaultProps> = ({ children }) => <>{children}</>;

interface SwitchProps {
    children: React.ReactElement<CaseProps | DefaultProps>[];
}

const Switch: React.FC<SwitchProps> = ({ children }) => {
    let defaultChild = null;

    const result = React.Children.toArray(children).find((child) => {
        if (React.isValidElement(child) && child.props.condition !== undefined) {
            return child.props.condition;
        }
        if (React.isValidElement(child) && child.type === Default) {
            defaultChild = child;
            return false;
        }
        return false;
    });

    return <>{result || defaultChild}</>;
};

export { Case, Default, Switch };

