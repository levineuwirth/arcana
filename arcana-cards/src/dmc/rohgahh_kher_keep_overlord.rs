//! Rohgahh, Kher Keep Overlord — `{3}{B}{R}` 4/4 Legendary Kobold Warrior (B/R).
//! Anthem static "Other Kobolds you control get +2/+2" is GAP'd.
//! Whenever you cast a Kobold spell, you may pay {2} → create a 4/4 red Dragon w/ flying.
//! Whenever you cast a Dragon spell, create a 0/1 red Kobold "Kobolds of Kher Keep".

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Rohgahh, Kher Keep Overlord");
    let kobold = reg.interner_mut().intern("Kobold");
    let warrior = reg.interner_mut().intern("Warrior");
    let dragon = reg.interner_mut().intern("Dragon");
    let _kk_name = reg.interner_mut().intern("Kobolds of Kher Keep");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kobold);
    subtypes.0.insert(warrior);

    // GAP: static anthem "Other Kobolds you control get +2/+2."

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let kobold_spell = ObjectFilter::new().with_subtype_sym(kobold);
    let dragon_spell = ObjectFilter::new().with_subtype_sym(dragon);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(kobold_spell),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: maybe_make_dragon,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(dragon_spell),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_kobold,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn maybe_make_dragon(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dragon = reg.interner().lookup("Dragon").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    let token = TokenDefinition {
        name: dragon,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}").expect("valid cost")),
        then: Box::new(Effect::CreateToken { controller: trig.controller, token }),
        else_effect: None,
    }]
}

fn make_kobold(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let kk = reg.interner().lookup("Kobolds of Kher Keep").unwrap_or_default();
    let kobold = reg.interner().lookup("Kobold").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kobold);
    let token = TokenDefinition {
        name: kk,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
