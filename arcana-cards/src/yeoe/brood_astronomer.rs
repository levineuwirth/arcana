//! Brood Astronomer — `{1}{G}` 2/2 green Insect Scientist.
//!
//! "When this creature enters, you may sacrifice a land. If you do, draft a card
//!  from the Planets Spellbook and put it onto the battlefield tapped.
//!  {T}: Add one mana of any color. If you control a Planet with twelve or more
//!  charge counters on it, add three mana of any one color instead."
//!
//! Both abilities rely on mechanics with no engine model:
//!
//! * The ETB drafts a card from the "Planets Spellbook" — Spellbook drafting is
//!   not modeled (no Effect for it; Planets are an Unfinity-style supplemental
//!   deck) — GAP'd.
//! * The mana ability adds "one mana of ANY COLOR" (a player color choice, with
//!   a Planet-charge-counter conditional). `ManaUnit::plain` requires a fixed
//!   color and there is no any-color / choose-a-color mana primitive, nor a
//!   "Planet" type to test — so the activated mana ability is GAP'd.

// GAP (trigger): "you may sacrifice a land. If you do, draft a card from the
// Planets Spellbook and put it onto the battlefield tapped." — Spellbook
// drafting is not modeled.
// GAP (ability): "{T}: Add one mana of any color. If you control a Planet with
// twelve or more charge counters on it, add three mana of any one color
// instead." — no any-color/choose-color mana primitive (ManaUnit::plain needs a
// fixed color), and no "Planet" type to test the conditional.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brood Astronomer");
    let insect = reg.interner_mut().intern("Insect");
    let scientist = reg.interner_mut().intern("Scientist");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(scientist);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_draft_planet,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_draft_planet(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: drafting from the Planets Spellbook is not modeled.
    Vec::new()
}
