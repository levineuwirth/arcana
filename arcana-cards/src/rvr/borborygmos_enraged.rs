//! Borborygmos Enraged — `{4}{R}{R}{G}{G}` 7/6 Legendary Cyclops (G/R).
//! Trample.
//! "Whenever Borborygmos Enraged deals combat damage to a player, reveal the
//! top three cards of your library. Put all land cards revealed this way into
//! your hand and the rest into your graveyard."
//! "Discard a land card: Borborygmos Enraged deals 3 damage to any target."
//!
//! Trample is a base keyword. The combat-damage trigger's effect (reveal top 3,
//! lands to hand, rest to graveyard) has no matching Effect — DigTopN is a
//! single-take and RevealUntil stops at the first match — so the effect is
//! GAP'd. The activated ability discards a land card to deal 3 to any target.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Borborygmos Enraged");
    let cyclops = reg.interner_mut().intern("Cyclops");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cyclops);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}{G}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: reveal_three_lands,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Discard a land card: Borborygmos Enraged deals 3 damage \
                       to any target."
                    .into(),
                cost: ActivationCost {
                    discard_other: Some(
                        ObjectFilter::new().with_types(TypeLine::LAND.into()),
                    ),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_three_any,
            }),
    )
}

fn reveal_three_lands(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal the top three cards, put all land cards into your hand and
    // the rest into your graveyard" — no Effect reveals a fixed count and
    // partitions lands→hand / rest→graveyard (DigTopN is single-take).
    Vec::new()
}

fn deal_three_any(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 3,
    }]
}
