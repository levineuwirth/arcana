//! Rafi, Retro Racer — `{3}{R}` 3/3 Legendary Dinosaur Pilot (red).
//!
//! * "When Rafi enters and at the beginning of the first upkeep in a game
//!   where it's your commander, there's mana burn and combat damage uses the
//!   stack for the rest of the game." — GAP: a permanent game-rule alteration
//!   (mana burn + combat damage on the stack); not expressible as an effect.
//!   The ETB half is registered as a trigger shell with a GAP'd no-op; the
//!   commander-upkeep half has no matching trigger and is omitted.
//! * "{R}, Sacrifice Rafi: Each player adds {R}{R}{R}." — activated ability:
//!   mana cost {R} + sacrifice self; each player gains three red mana.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::script;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rafi, Retro Racer");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let pilot = reg.interner_mut().intern("Pilot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(pilot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: gap_rules_alteration,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}, Sacrifice Rafi: Each player adds {R}{R}{R}.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: each_player_adds_rrr,
            }),
    )
}

fn gap_rules_alteration(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "there's mana burn and combat damage uses the stack for the rest
    // of the game" — a permanent game-rule alteration; not expressible.
    Vec::new()
}

fn each_player_adds_rrr(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let players = script::all_players(state);
    let mut effects = Vec::new();
    for p in players {
        effects.push(Effect::AddMana {
            player: p,
            mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source); 3],
        });
    }
    vec![Effect::Sequence(effects)]
}
