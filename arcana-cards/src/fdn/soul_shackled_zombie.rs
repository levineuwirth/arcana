//! Soul-Shackled Zombie — `{3}{B}` 4/2 black Zombie.
//! "When this creature enters, exile up to two target cards from a
//! single graveyard. If at least one creature card was exiled this
//! way, each opponent loses 2 life and you gain 2 life."
//! GAP: "from a single graveyard" constraint on multiple targets not
//! expressible; "if at least one creature card" conditional on what
//! was exiled not expressible. Emitting exile for both targets and
//! unconditional life effects.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul-Shackled Zombie");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new(),
                    },
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
            }),
    )
}

fn etb_exile_graveyard(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = trig
        .targets
        .targets
        .iter()
        .filter_map(|t| {
            if let TargetChoice::Object(id) = t {
                Some(Effect::ExileFromGraveyard { target: *id })
            } else {
                None
            }
        })
        .collect();
    // GAP: "if at least one creature card" — conditional not expressible;
    // emitting life effects unconditionally
    let opponents = script::opponents(state, trig.controller);
    for p in opponents {
        effects.push(Effect::LoseLife { player: p, amount: 2 });
    }
    effects.push(Effect::GainLife { player: trig.controller, amount: 2 });
    effects
}
