//! Amalia Benavides Aguirre — `{W}{B}` 2/2 Legendary Vampire Scout.
//! Ward—Pay 3 life. "Whenever you gain life, Amalia explores. Then
//! destroy all other creatures if its power is exactly 20."
//!
//! The life-gain trigger fires the explore. The "destroy all if its
//! power is exactly 20" rider is a power-self conditional with no
//! demonstrated primitive and is GAP'd. Ward—Pay 3 life is a non-mana
//! ward cost (not expressible) so the Ward keyword is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Amalia Benavides Aguirre");
    let vampire = reg.interner_mut().intern("Vampire");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Ward—Pay 3 life is a non-mana ward cost; not expressible.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::LifeGained {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_life_gained,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_life_gained(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Amalia explores."
    // GAP: "Then destroy all other creatures if its power is exactly 20"
    // is a self-power conditional with no demonstrated primitive.
    vec![Effect::Explore {
        player: trig.controller,
        target: trig.source,
    }]
}
