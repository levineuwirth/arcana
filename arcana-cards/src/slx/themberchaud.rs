//! Themberchaud — `{4}{R}{R}{R}` 5/5 Legendary Dragon with Trample.
//!
//! * Trample — keyword line (Exert is not a usable `KeywordAbility`
//!   variant; see the GAP below).
//! * "When Themberchaud enters, he deals X damage to each other creature
//!   without flying and each player, where X is the number of Mountains
//!   you control." → a `SelfEntersBattlefield` trigger: X = count of
//!   Mountains you control (`script::count_matching` over a Mountain
//!   subtype filter), then `Effect::ForEach` deals X to each non-flying
//!   creature (excluding Themberchaud) and X to each player.
//! * GAP: "You may exert Themberchaud as he attacks. When you do, he
//!   gains flying until end of turn." — exert (the attack-time may-exert
//!   choice + its 'when you do' rider) is not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Themberchaud");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_blast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_blast(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // X = number of Mountains you control.
    let x = script::count_matching(
        state,
        &script::subtype_filter(reg, "Mountain")
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );

    // X damage to each OTHER creature without flying.
    let creatures: Vec<_> = script::ids_matching(
        state,
        &arcana_core::targets::ObjectFilter::creature()
            .without_keyword(KeywordAbility::Flying),
        trig.controller,
    )
    .into_iter()
    .filter(|&id| id != trig.source)
    .collect();

    let mut effects = vec![Effect::ForEach {
        targets: creatures,
        effect: Box::new(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: x,
        }),
    }];

    // X damage to each player.
    for p in script::all_players(state) {
        effects.push(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(p),
            amount: x,
        });
    }

    effects
}
