//! Invasion of Amonkhet // Lazotep Convert — `{1}{U}{B}` Battle — Siege.
//! Enters with 4 defense counters.
//! ETB: each player mills 3, then each opponent discards 1, you draw 1.
//!
//! Back face: Creature — Zombie 4/4.
//! "You may have this creature enter as a copy of any creature card in a
//!  graveyard, except it's a 4/4 black Zombie in addition to its other colors
//!  and types."
//!
//! Defeat-transform to the 4/4 Zombie back face is auto-wired by the engine SBA
//! (CR 310.11); the back face's printed 4/4 black Zombie body is its modeled form.
//!
//! # GAP notes
//! - GAP: back face ETB "you may have this creature enter as a copy of any
//!   creature card in a graveyard (except it's a 4/4 black Zombie too)" is a
//!   copy/replacement effect with a choose-from-graveyard selection; no Effect
//!   variant copies an arbitrary graveyard card with stat/type overrides onto an
//!   already-on-battlefield permanent. The back face stays a plain 4/4 Zombie.
//!   (Note: on defeat the creature is already on the battlefield, so the
//!   enters-as-a-copy replacement would not even apply — it only matters when the
//!   card is cast as the creature, which the engine does not do for Sieges.)

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Amonkhet");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Lazotep Convert — 4/4 Zombie.
    let back_name = reg.interner_mut().intern("Lazotep Convert");
    let zombie_sub = reg.interner_mut().intern("Zombie");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(zombie_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 4,
            })
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
    )
}

fn etb_trigger(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();

    // Each player mills 3.
    for p in script::all_players(state) {
        effects.push(Effect::Mill { player: p, count: 3 });
    }

    // Each opponent discards 1.
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::Discard {
            player: opp,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
    }

    // You draw 1.
    effects.push(Effect::DrawCards { player: trig.controller, count: 1 });

    effects
}
