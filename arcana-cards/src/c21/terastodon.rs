//! Terastodon — `{6}{G}{G}` 9/9 green Creature — Elephant.
//! "When this creature enters, you may destroy up to three target
//! noncreature permanents. For each permanent put into a graveyard
//! this way, its controller creates a 3/3 green Elephant creature token."
//!
//! GAP: "its controller creates a token" — the engine cannot track
//! which controller owned each destroyed permanent at resolve time;
//! tokens are created for the triggering player instead.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Terastodon");
    let elephant = reg.interner_mut().intern("Elephant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(9)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_destroy_noncreatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .without_types(TypeLine::CREATURE.into()),
                    ),
                    count: TargetCount::UpTo(3),
                    controller: None,
                }],
            }),
    )
}

fn etb_destroy_noncreatures(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elephant = reg.interner().lookup("Elephant")
        .expect("Elephant interned during register()");
    let mut effects = Vec::new();
    for target in &trig.targets.targets {
        let TargetChoice::Object(id) = target else { continue; };
        effects.push(Effect::DestroyPermanent { target: *id });
        // GAP: should create token for the destroyed permanent's controller,
        // but per-target controller is not accessible here; using trig.controller.
        let mut token_subtypes = SubtypeSet::default();
        token_subtypes.0.insert(elephant);
        effects.push(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: elephant,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes: token_subtypes,
                power: Some(PtValue::Fixed(3)),
                toughness: Some(PtValue::Fixed(3)),
                keywords: vec![],
                abilities: vec![],
            },
        });
    }
    effects
}
