//! Ainok Strike Leader — `{1}{W}` 2/2 white Dog Warrior.
//! "Whenever you attack with this creature and/or your commander, for
//! each opponent, create a 1/1 red Goblin creature token that's tapped
//! and attacking that player."
//! "Sacrifice this creature: Creature tokens you control gain
//! indestructible until end of turn."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ainok Strike Leader");
    let dog = reg.interner_mut().intern("Dog");
    let warrior = reg.interner_mut().intern("Warrior");
    let _goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // "Whenever you attack with this creature [and/or your commander]…"
            // GAP: the "and/or your commander" clause (commander identity) isn't
            // modeled; we fire on this creature attacking.
            // GAP: tokens are created untapped and not attacking — no
            // tapped-and-attacking token primitive in the demonstrated API.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_make_goblins,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "Sacrifice this creature: Creature tokens you control gain
            // indestructible until end of turn."
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice this creature: Creature tokens you control gain \
                       indestructible until end of turn."
                    .into(),
                cost: ActivationCost { sacrifice: true, ..ActivationCost::default() },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tokens_gain_indestructible,
            }),
    )
}

fn attack_make_goblins(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let goblin = reg.interner().lookup("Goblin").unwrap_or_default();
    let opponents = script::opponents(state, trig.controller);
    opponents
        .into_iter()
        .map(|_| {
            let mut subtypes = SubtypeSet::default();
            subtypes.0.insert(goblin);
            Effect::CreateToken {
                controller: trig.controller,
                token: TokenDefinition {
                    name: goblin,
                    colors: ColorSet::red(),
                    types: TypeLine::CREATURE.into(),
                    subtypes,
                    power: Some(PtValue::Fixed(1)),
                    toughness: Some(PtValue::Fixed(1)),
                    keywords: vec![],
                    abilities: vec![],
                },
            }
        })
        .collect()
}

fn tokens_gain_indestructible(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .tokens_only();
    let ids = script::ids_matching(state, &filter, ctx.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::GrantKeyword {
            target: NULL_OBJECT_ID,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        }),
    }]
}
