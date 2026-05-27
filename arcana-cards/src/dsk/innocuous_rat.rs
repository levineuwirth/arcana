//! Innocuous Rat — `{1}{B}` 1/1 black Rat.
//! "When this creature dies, manifest dread. (Look at the top two cards of your
//! library. Put one onto the battlefield face down as a 2/2 creature and the other
//! into your graveyard.)"
//! GAP: Manifest Dread is not the same as Manifest — it involves looking at two
//! cards and putting one into graveyard. Effect::Manifest only does standard manifest.
//! Using Manifest as best effort (over-simplified).

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
    let name = reg.interner_mut().intern("Innocuous Rat");
    let rat = reg.interner_mut().intern("Rat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_manifest_dread,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_manifest_dread(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Manifest Dread (look at top 2, put one face-down, other to graveyard)
    // differs from standard Manifest (just top card face down). Using Manifest
    // as closest available primitive.
    vec![Effect::Manifest { player: trig.controller }]
}
