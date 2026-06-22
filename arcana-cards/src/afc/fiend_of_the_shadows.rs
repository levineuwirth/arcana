//! Fiend of the Shadows — `{3}{B}{B}` 3/3 Vampire Wizard.
//!
//! Flying
//! Whenever this creature deals combat damage to a player, that player exiles
//! a card from their hand. You may play that card for as long as it remains
//! exiled.
//! Sacrifice a Human: Regenerate this creature.
//!
//! The combat-damage trigger is wired (self-source idiom, target a player).
//! Its payoff — the damaged player exiles a card from their hand that you may
//! then play — has no expressible primitive (no "opponent exiles from hand,
//! you may play it"), so the body is a documented GAP. The sacrifice-a-Human
//! regenerate activated ability IS wired via `sacrifice_other`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fiend of the Shadows");
    let vampire = reg.interner_mut().intern("Vampire");
    let wizard = reg.interner_mut().intern("Wizard");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let sac_human = ObjectFilter::creature().with_subtype_sym(human);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: exile_from_hand_play,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice a Human: Regenerate this creature.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(sac_human),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: regenerate_self,
            }),
    )
}

fn exile_from_hand_play(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "that player exiles a card from their hand. You may play that card
    // for as long as it remains exiled." — no opponent-exiles-from-hand-and-
    // you-may-play primitive.
    Vec::new()
}

fn regenerate_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Regenerate { target: ctx.source }]
}
