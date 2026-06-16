//! Cadric, Soul Kindler — `{2}{R}{W}` 4/3 Legendary Creature — Dwarf Wizard.
//! "The legend rule doesn't apply to tokens you control."
//! "Whenever another nontoken legendary permanent you control enters, you may
//! pay {1}. If you do, create a token that's a copy of it. That token gains
//! haste. Sacrifice it at the beginning of the next end step."

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cadric, Soul Kindler");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "The legend rule doesn't apply to tokens you control." — a
    // rule-replacement static with no trigger/cost; not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::permanent()
                    .controlled_by(ControllerConstraint::You)
                    .nontoken()
                    .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: maybe_copy_legend,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn maybe_copy_legend(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.entering_object() else {
        return Vec::new();
    };
    // GAP: the created token's "gains haste" and "sacrifice at the beginning of
    // the next end step" riders can't be attached — the copy token's id is
    // minted inside CopyPermanent and isn't available to schedule against.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}").expect("valid cost")),
        then: Box::new(Effect::CopyPermanent { target: id }),
        else_effect: None,
    }]
}
