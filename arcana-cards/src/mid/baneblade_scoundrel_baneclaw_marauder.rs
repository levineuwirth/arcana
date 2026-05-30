//! Baneblade Scoundrel // Baneclaw Marauder — `{3}{B}` Human Rogue Werewolf 4/3.
//! Front: Whenever this creature becomes blocked, each creature blocking it gets -1/-1 until end of turn.
//! Front: Daybound (day/night cycle — GAP: not modeled).
//! Back (Baneclaw Marauder): Werewolf.
//! Back: Whenever this creature becomes blocked, each creature blocking it gets -1/-1 until end of turn.
//! Back: Whenever a creature blocking this creature dies, that creature's controller loses 1 life.
//! Back: Nightbound (GAP: day/night cycle not modeled).
//!
//! GAP: Daybound/Nightbound day/night cycle mechanics are not modeled.
//! No transform trigger is wired (day/night transformation has no expressible TriggerCondition).
//! GAP: Back-face "blocking creature dies -> controller loses 1 life" is a back-face-only
//! triggered ability not modeled.
//! The "becomes blocked" trigger fires on both faces (shared ability modeled on front).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Baneblade Scoundrel");
    let human_sub = reg.interner_mut().intern("Human");
    let rogue_sub = reg.interner_mut().intern("Rogue");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(rogue_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Baneclaw Marauder");
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // "Whenever this creature becomes blocked, each creature blocking it gets -1/-1 until end of turn."
            // Modeled on front face; fires on both faces (shared combat trigger).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                intervening_if: None,
                effect: becomes_blocked_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
        // GAP: Daybound/Nightbound day/night cycle transform not wired (no TriggerCondition available).
        // GAP: Back-face "whenever a creature blocking this dies" triggered ability not modeled.
    )
}

fn becomes_blocked_trigger(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Each creature blocking this creature gets -1/-1 until end of turn.
    // We need to find all creatures that are currently blocking trig.source.
    // There is no script helper for "creatures blocking a specific creature".
    // Use ids_matching for creatures controlled by any opponent — but we can't
    // filter to "blocking this" specifically. GAP: use a board-wide creature
    // filter and rely on the game engine to narrow by combat state.
    // Best effort: pump all opposing creatures -1/-1 is WRONG (too broad).
    // GAP: "each creature blocking it" requires a combat state query not available
    // via script::ids_matching. Emitting Vec::new() to avoid a materially wrong card.
    // GAP: "each creature blocking this" — combat blocker filter not expressible.
    let _ = (state, trig);
    Vec::new()
}
