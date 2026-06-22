//! Enigma Eidolon — `{3}{U}` 2/2 blue Spirit.
//!
//! * "{U}, Sacrifice this creature: Target player mills three cards." — an
//!   activated ability with a mana + sacrifice-self cost targeting a player.
//! * "Whenever you cast a multicolored spell, you may return this card from
//!   your graveyard to your hand." — a `SpellCast` trigger (caster = you).
//!   The "multicolored" filter is not expressible with the demonstrated
//!   `ObjectFilter` refinements (no multicolor predicate), so the trigger is
//!   left unfiltered and the multicolor restriction is GAP'd. The return is
//!   from the graveyard, so the trigger lives in `Zone::Graveyard(0)`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Enigma Eidolon");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}, Sacrifice this creature: Target player mills three cards.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: mill_target_player,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "multicolored spell" — no multicolor predicate on
                // ObjectFilter; trigger left unfiltered (fires on any spell you cast).
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: return_self_from_graveyard,
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn mill_target_player(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![Effect::Mill { player: *p, count: 3 }]
}

/// "you may return this card from your graveyard to your hand" — the "may"
/// is a resolution-time choice; we apply the return of the source card.
fn return_self_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToHand { target: trig.source }]
}
