//! Necron Deathmark — `{3}{B}{B}` 5/3 Artifact Creature — Necron with Flash.
//! "Synaptic Disintegrator — When this creature enters, destroy up to one
//!  target creature and target player mills three cards."
//!
//! Flash is a base characteristic. The named ETB ability is one trigger with
//! two target requirements: an optional (up-to-one) target creature to destroy
//! and a target player to mill three. The resolver dispatches by TargetChoice
//! variant so it's robust whether or not the optional creature was chosen.
//! ("Synaptic Disintegrator" and "Mill" are flavor/ability names, not keyword
//! abilities — only Flash maps to a KeywordAbility variant.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Necron Deathmark");
    let necron = reg.interner_mut().intern("Necron");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(necron);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: synaptic_disintegrator,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
                TargetRequirement::target_player(),
            ],
        }),
    )
}

fn synaptic_disintegrator(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for choice in &trig.targets.targets {
        match choice {
            TargetChoice::Object(id) => {
                effects.push(Effect::DestroyPermanent { target: *id });
            }
            TargetChoice::Player(p) => {
                effects.push(Effect::Mill { player: *p, count: 3 });
            }
            _ => {}
        }
    }
    effects
}
