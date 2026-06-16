//! Zareth San, the Trickster — `{3}{U}{B}` 4/4 Legendary Merfolk Rogue with Flash.
//!
//! Oracle:
//! * Flash
//! * {2}{U}{B}, Return an unblocked attacking Rogue you control to its owner's
//!   hand: Put this card from your hand onto the battlefield tapped and attacking.
//! * Whenever Zareth San deals combat damage to a player, you may put target
//!   permanent card from that player's graveyard onto the battlefield under
//!   your control.
//!
//! GAPs / fidelity notes:
//! * The activated ability's additional cost "Return an unblocked attacking
//!   Rogue you control to its owner's hand" has no matching `ActivationCost`
//!   field (the catalog covers sacrifice/tap/discard, not bounce-a-creature),
//!   so only the mana cost is modeled; the bounce cost is GAP'd. The effect
//!   (put this from hand tapped and attacking) is faithful, activated from Hand.
//! * The combat-damage trigger targets a permanent card in a graveyard and
//!   returns it under YOUR control via ReturnFromGraveyardToBattlefield +
//!   ChangeControl. Restricting the graveyard to "that player's" specifically
//!   (the damaged player) is not expressible — the target zone player index is
//!   fixed — so the filter is "a permanent card in a graveyard" (fidelity gap).
//!   "You may" is a resolution-time optional choice and is not separately gated.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zareth San, the Trickster");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}{B}, Return an unblocked attacking Rogue you control \
                       to its owner's hand: Put this card from your hand onto the \
                       battlefield tapped and attacking."
                    .into(),
                // GAP: the "Return an unblocked attacking Rogue you control to its
                // owner's hand" additional cost is not expressible as an
                // ActivationCost field; only the mana cost is modeled.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Hand,
                is_instant_speed: false,
                face_gate: None,
                effect: put_self_tapped_attacking,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter { name: Some(name), ..ObjectFilter::default() },
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: reanimate_from_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::permanent(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn put_self_tapped_attacking(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "Put this card from your hand onto the battlefield tapped and attacking."
    let nm = reg.interner().lookup("Zareth San, the Trickster");
    vec![Effect::PutFromHandOntoBattlefieldTappedAttacking {
        player: ctx.controller,
        filter: ObjectFilter { name: nm, ..ObjectFilter::default() },
    }]
}

fn reanimate_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
        Effect::ChangeControl { target: *id, new_controller: trig.controller },
    ]
}
