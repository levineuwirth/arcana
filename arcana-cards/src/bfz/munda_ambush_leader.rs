//! Munda, Ambush Leader — `{2}{R}{W}` 3/4 Legendary Creature — Kor Ally.
//!
//! * Haste (keyword).
//! * "Rally — Whenever Munda or another Ally you control enters, you may look at
//!   the top four cards of your library. If you do, reveal any number of Ally
//!   cards from among them, then put those cards on top of your library in any
//!   order and the rest on the bottom in any order." — modeled as a ZoneChange
//!   trigger (an Ally you control enters). GAP: the effect is a multi-card
//!   reveal-and-reorder (look at 4, take ANY NUMBER of Allies to the top in any
//!   order); `DigTopN` is single-take and there is no multi-card top-stack
//!   primitive, so the effect body is left empty.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::script;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Munda, Ambush Leader");
    let kor = reg.interner_mut().intern("Kor");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(ally);

    let ally_filter =
        script::subtype_filter(reg, "Ally").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ally_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: rally_reveal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn rally_reveal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at top 4, reveal ANY NUMBER of Ally cards, put those on top in
    // any order and the rest on the bottom" — a multi-card optional take +
    // reorder. DigTopN is single-take only; no top-stack-reorder primitive.
    Vec::new()
}
