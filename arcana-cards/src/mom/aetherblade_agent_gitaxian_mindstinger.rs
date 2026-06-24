//! Aetherblade Agent // Gitaxian Mindstinger — `{1}{B}` Human Rogue 1/1 with Deathtouch (front).
//! {4}{U/P}: Transform this creature. Activate only as a sorcery.
//! ({U/P} can be paid with either {U} or 2 life.)
//! Back face (Gitaxian Mindstinger): Phyrexian Rogue with Deathtouch.
//! Whenever this creature deals combat damage to a player or battle, draw a card.
//!
//! GAP: {U/P} hybrid-Phyrexian mana cost. The engine's ManaCost::parse supports
//! {U/P} notation; parsed as best-effort. The "Activate only as a sorcery" restriction
//! is enforced via is_instant_speed: false.
//! GAP: the oracle's "or battle" half of the back-face combat-damage trigger isn't
//! expressible (no battle-damage target filter); the "to a player" half is wired.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aetherblade Agent");
    let human_sub = reg.interner_mut().intern("Human");
    let rogue_sub = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(rogue_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Gitaxian Mindstinger");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let back_rogue_sub = reg.interner_mut().intern("Rogue");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(phyrexian_sub);
    back_subtypes.0.insert(back_rogue_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Deathtouch],
            ..Default::default()
        },
        spell_ability: None,
    };

    // Restrict the back-face combat-damage trigger's source to Gitaxian
    // Mindstinger itself by name (the unique back face that printed it).
    let back_self_name = reg.interner().lookup("Gitaxian Mindstinger");
    let back_self_filter = ObjectFilter { name: back_self_name, ..ObjectFilter::default() };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {4}{U/P}: Transform. Sorcery speed only. Face-gated to front face.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{U/P}: Transform this creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{U/P}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_self,
            })
            // Back face (Gitaxian Mindstinger): "Whenever this creature deals combat
            // damage to a player or battle, draw a card." The "to a player" half is
            // wired; the "or battle" half is a GAP (no battle-damage target filter).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: back_self_filter,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: back_draw_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 1),
    )
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

fn back_draw_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
