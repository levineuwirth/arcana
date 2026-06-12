//! Plaguecrafter — `{2}{B}` 3/2 black Human Shaman.
//! "When this creature enters, each player sacrifices a creature or
//! planeswalker of their choice. Each player who can't discards a card."
//!
//! "Each player sacrifices a creature or planeswalker of their choice" —
//! one `Effect::ChooseNFromZone` per player (chooser = that player,
//! creature-or-planeswalker via `with_types_any`, action Sacrifice).
//! GAP: "Each player who can't discards a card" — the conditional discard
//! fallback (fires only for players with no creature or planeswalker) is
//! not expressible; the sacrifice half is wired, the fallback is omitted.

use arcana_core::effects::{Effect, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Plaguecrafter");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_each_sac,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_each_sac(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Each player sacrifices a creature or planeswalker of their choice"
    // — one ChooseNFromZone per player; the controller constraint is
    // evaluated from the CHOOSER's perspective. Separate top-level
    // effects so each pending choice parks correctly.
    // GAP: "Each player who can't discards a card" — conditional discard
    // fallback not expressible.
    let filter = ObjectFilter::permanent()
        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER))
        .controlled_by(ControllerConstraint::You);
    script::all_players(state)
        .into_iter()
        .map(|p| Effect::ChooseNFromZone {
            chooser: p,
            zone: Zone::Battlefield,
            filter: filter.clone(),
            min: 1,
            max: 1,
            action: PickAction::Sacrifice,
        })
        .collect()
}
