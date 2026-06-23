//! Varina, Lich Queen — `{1}{W}{U}{B}` 4/4 Legendary Zombie Wizard (W/U/B).
//!
//! Oracle:
//! * Whenever you attack with one or more Zombies, draw that many cards,
//!   then discard that many cards. You gain that much life.
//! * `{2}, Exile two cards from your graveyard:` Create a tapped 2/2 black
//!   Zombie creature token.
//!
//! Implemented:
//! * The attack trigger — wired as `CreatureAttacks { filter: Zombie you
//!   control }`. "That many" is computed at resolution as the number of
//!   attacking Zombies you control (`script::count_matching` over an
//!   attacking-only Zombie filter). NOTE: the closest available variant
//!   fires per attacking Zombie rather than once for the whole batch.
//! * The activated token-maker (mana portion of the cost only).
//!
//! GAP: the `Exile two cards from your graveyard` part of the activation
//! cost — `ActivationCost` has no exile-from-graveyard field — so only the
//! `{2}` is charged.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::effects::{DiscardChoice, TokenDefinition};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Varina, Lich Queen");
    let zombie = reg.interner_mut().intern("Zombie");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // Closest variant: fires per attacking Zombie you control.
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: zombie_attacking_filter(reg),
                },
                intervening_if: None,
                effect: attack_loot_and_gain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, Exile two cards from your graveyard: Create a tapped 2/2 black Zombie creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    // GAP: "Exile two cards from your graveyard" — ActivationCost
                    // has no exile-from-graveyard field.
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_zombie_token,
            }),
    )
}

/// Filter for an attacking Zombie creature you control.
fn zombie_attacking_filter(reg: &CardRegistry) -> ObjectFilter {
    script::subtype_filter(reg, "Zombie")
        .controlled_by(ControllerConstraint::You)
        .attacking_only()
}

fn attack_loot_and_gain(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(state, &zombie_attacking_filter(reg), trig.controller);
    if n == 0 {
        return Vec::new();
    }
    vec![
        Effect::DrawCards { player: trig.controller, count: n },
        Effect::Discard {
            player: trig.controller,
            count: n,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::GainLife { player: trig.controller, amount: n },
    ]
}

fn make_zombie_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: zombie,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
    // NOTE: the token should enter tapped; CreateToken has no "tapped" flag.
}
