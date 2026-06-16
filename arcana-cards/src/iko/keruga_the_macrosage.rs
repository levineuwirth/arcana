//! Keruga, the Macrosage — `{3}{G/U}{G/U}` 5/4 Legendary Dinosaur Hippo.
//! Companion (deck-building keyword — GAP).
//! When Keruga enters, draw a card for each other permanent you control with
//! mana value 3 or greater.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Keruga, the Macrosage");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let hippo = reg.interner_mut().intern("Hippo");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(hippo);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G/U}{G/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Companion is a deck-building keyword, not in the usable surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: draw_for_big_permanents,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn draw_for_big_permanents(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "for each other permanent you control with mana value 3 or greater".
    // GAP: the "other" exclusion of Keruga itself is not expressible in the
    // filter; the count may include Keruga (mv 5 >= 3).
    let n = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .controlled_by(ControllerConstraint::You)
            .with_min_cmc(3),
        trig.controller,
    );
    vec![Effect::DrawCards {
        player: trig.controller,
        count: n,
    }]
}
