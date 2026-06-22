//! Glorious Protector — `{2}{W}{W}` 3/4 Angel Cleric with Flash and Flying.
//! Flash, Flying.
//! When this creature enters, you may exile any number of non-Angel creatures
//! you control until this creature leaves the battlefield.
//! Foretell {2}{W}.
//!
//! Flash + Flying are wired. The ETB exiles any number of non-Angel creatures
//! you control (ChooseAnyNumberFromZone + Exile). GAP: the "until this creature
//! leaves the battlefield" return linkage can't be combined with the
//! variable-count pick (ExileUntilSourceLeaves is single-target only). Foretell
//! is GAP'd — no KeywordAbility::Foretell variant.

use arcana_core::effects::{Effect, KeywordAbility, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glorious Protector");
    let angel = reg.interner_mut().intern("Angel");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_exile_non_angels,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_exile_non_angels(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    if let Some(angel) = reg.interner().lookup("Angel") {
        filter = filter.without_subtype_sym(angel);
    }
    // GAP: "until this creature leaves the battlefield" return linkage is not
    // expressible for a variable-count pick — this exiles the chosen creatures
    // permanently.
    vec![Effect::ChooseAnyNumberFromZone {
        chooser: trig.controller,
        zone: Zone::Battlefield,
        filter,
        action: PickAction::Exile,
    }]
}
