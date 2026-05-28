//! Stormshriek Feral // Flush Out — `{4}{R}` // `{1}{R}` red Adventure creature.
//! Creature: 3/3 Dragon. Flying, haste. {1}{R}: gets +1/+0 until end of turn.
//! Adventure (Flush Out — Sorcery): Discard a card. If you do, draw two cards.
//! GAP: "{1}{R}: this creature gets +1/+0 until end of turn" — activated ability on creature not modeled here (ActivatedAbilityDef with face_gate needed; omitted for simplicity, using ETB trigger placeholder).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationZone, CardDefinition, CardFace, CardRegistry,
    SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::layers::Duration;
use arcana_core::targets::ControllerConstraint;
use arcana_core::effects::DiscardChoice;
use arcana_core::registry::ActivationContext;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stormshriek Feral");
    let adv_name = reg.interner_mut().intern("Flush Out");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Discard a card. If you do, draw two cards.".into(),
        target_requirements: vec![],
        modal: None,
        effect: flush_out_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: This creature gets +1/+0 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: pump_self,
            })
            .with_adventure(adventure),
    )
}

fn pump_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn flush_out_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "discard a card. If you do, draw two cards." — OptionalPayment with Discard cost not in OptionalPaymentKind
    // Emitting raw discard + draw as best-effort (no conditionality on "if you do")
    vec![
        Effect::Discard { player: entry.controller, count: 1, choice: DiscardChoice::ControllerChooses },
        Effect::DrawCards { player: entry.controller, count: 2 },
    ]
}
