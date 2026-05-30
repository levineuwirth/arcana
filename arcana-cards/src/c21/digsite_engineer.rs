//! Digsite Engineer — `{2}{W}` 3/3 white Dwarf Artificer.
//! "Whenever you cast an artifact spell, you may pay {2}. If you do,
//! create a 0/0 colorless Construct artifact creature token with
//! 'This token gets +1/+1 for each artifact you control.'"
//! GAP: the token's static ability "+1/+1 for each artifact you control"
//! is not expressible via TokenDefinition (no static-ability field).
//! The token is created as a plain 0/0 Construct artifact creature.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Digsite Engineer");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let artificer = reg.interner_mut().intern("Artificer");
    let _construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter {
                        types: Some(TypeLine::ARTIFACT.into()),
                        ..ObjectFilter::default()
                    }),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: maybe_create_construct,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn maybe_create_construct(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let construct = reg.interner().lookup("Construct")
        .expect("Construct interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(construct);
    let token = TokenDefinition {
        name: construct,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}").expect("valid cost")),
        then: Box::new(Effect::CreateToken {
            controller: trig.controller,
            token,
        }),
        else_effect: None,
    }]
}
