//! Taj-Nar Swordsmith — `{3}{W}` 2/3 Cat Soldier.
//! "When this creature enters, you may pay {X}. If you do, search
//! your library for an Equipment card with mana value X or less,
//! put that card onto the battlefield, then shuffle."
//!
//! GAP: "you may pay {X}" — optional X-mana payment at resolution
//! is not expressible via the effect catalog. Emitting unconditional
//! TutorToBattlefield for an Equipment card; the payment and MV-X
//! restriction cannot be modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Taj-Nar Swordsmith");
    let cat = reg.interner_mut().intern("Cat");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
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

fn on_etb(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may pay {X}" — optional mana payment not expressible.
    // GAP: Equipment-subtype filter not in ObjectFilter; using artifact
    // filter as approximation. GAP: MV ≤ X restriction not applicable.
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
        tapped: false,
    }]
}
