//! Person of Interest — `{3}{R}` 2/2 Human Rogue.
//! Keywords: Suspect (not in catalog — keywords: vec![])
//! "When this creature enters, suspect it. Create a 2/2 white and
//! blue Detective creature token."
//!
//! GAP: "suspect it" — the Suspect mechanic (gives menace, can't block)
//! is not in the catalog.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Person of Interest");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let _detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_etb(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let detective = reg
        .interner()
        .lookup("Detective")
        .expect("Detective interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(detective);
    let token = TokenDefinition {
        name: detective,
        colors: ColorSet(ColorSet::WHITE | ColorSet::BLUE),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "suspect it" — Suspect mechanic not in catalog.
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
