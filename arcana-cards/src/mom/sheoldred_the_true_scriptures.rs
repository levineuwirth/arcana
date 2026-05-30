//! Sheoldred // The True Scriptures — `{3}{B}{B}` black Legendary Creature
//! — Phyrexian Praetor 4/5. Transforms to Enchantment — Saga (back face).
//!
//! Front face — Sheoldred:
//! Menace.
//! When Sheoldred enters, each opponent sacrifices a nontoken creature or
//! planeswalker of their choice.
//! GAP: "sacrifice a nontoken creature or planeswalker of their choice" —
//!   Sacrifice effect requires a filter; combining creature-or-planeswalker
//!   (no single TargetFilter covers both) and "each opponent chooses" is
//!   not fully expressible. Modeled as each opponent sacrifices a creature
//!   (approximation — omits planeswalker option).
//! {4}{B}: Exile Sheoldred, then return it to the battlefield transformed
//!   under its owner's control. Activate only as a sorcery and only if an
//!   opponent has eight or more cards in their graveyard.
//! GAP: "only if an opponent has 8+ cards in their graveyard" precondition
//!   not expressible via min_self_counters (requires opponent state check).
//!   Modeled as always-available activation (GAP on the condition).
//!
//! Back face — The True Scriptures (Enchantment — Saga):
//! I — For each opponent, destroy up to one target creature or planeswalker
//!     that player controls.
//! GAP: back-face Saga chapter abilities not auto-installed on transform.
//!
//! II — Each opponent discards three cards, then mills three cards.
//! GAP: back-face-only triggered ability not auto-installed on transform.
//!
//! III — Put all creature cards from all graveyards onto the battlefield
//!       under your control. Exile this Saga, then return it to the
//!       battlefield (front face up).
//! GAP: back-face-only triggered ability not auto-installed on transform.
//! GAP: "put all creature cards from all graveyards" — Reanimate is
//!   non-targeted from one graveyard; mass-reanimate not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sheoldred");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let praetor_sub = reg.interner_mut().intern("Praetor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian_sub);
    subtypes.0.insert(praetor_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // Back face: The True Scriptures — Enchantment — Saga
    let back_name = reg.interner_mut().intern("The True Scriptures");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(saga_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB: each opponent sacrifices a nontoken creature or planeswalker.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // {4}{B}: Exile then return transformed.
            // GAP: "only if an opponent has 8+ cards in their graveyard" not
            // expressible as an ActivationCost precondition.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{B}: Exile Sheoldred, then return it to the battlefield transformed.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: activate_exile_transform,
            })
            // GAP: back-face Saga chapter I, II, III triggered abilities not
            // auto-installed on transform.
    )
}

fn etb_sacrifice(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: should trigger when Sheoldred specifically enters, not any creature.
    // GAP: "each opponent sacrifices a nontoken creature or planeswalker of
    // their choice" — Sacrifice takes a filter for nontoken creatures;
    // planeswalker option and "opponent chooses which" not fully expressible.
    // Modeled as each opponent sacrifices one nontoken creature.
    let opponents = script::opponents(state, trig.controller);
    let filter = ObjectFilter::creature().nontoken();
    let effects: Vec<Effect> = opponents.into_iter().map(|opp| {
        Effect::Sacrifice { player: opp, filter: filter.clone(), count: 1 }
    }).collect();
    vec![Effect::Sequence(effects)]
}

fn activate_exile_transform(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Exile Sheoldred and return it to the battlefield transformed.
    vec![
        Effect::ExilePermanent { target: ctx.source },
        Effect::ReturnFromExileToBattlefield { target: ctx.source },
        Effect::Transform { target: ctx.source },
    ]
}
