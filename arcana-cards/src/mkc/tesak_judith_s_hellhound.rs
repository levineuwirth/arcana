//! Tesak, Judith's Hellhound — `{3}{R}` 3/3 Legendary Elemental Dog.
//! * Unleash (keyword)
//! * "Other Dogs you control have unleash." (static GAP)
//! * "Creatures you control with counters on them have haste." (static GAP)
//! * "Whenever Tesak attacks, add {R} for each attacking creature." (triggered)
//!
//! GAPs:
//! * "Other Dogs you control have unleash." — a continuous keyword-granting
//!   static over other permanents; no static-grant primitive in this class.
//! * "Creatures you control with counters on them have haste." — likewise a
//!   filtered continuous keyword-granting static.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tesak, Judith's Hellhound");
    let elemental = reg.interner_mut().intern("Elemental");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Unleash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_add_red,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attacks_add_red(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &ObjectFilter::creature().attacking_only(),
        trig.controller,
    );
    if n == 0 {
        return Vec::new();
    }
    let mana = vec![ManaUnit::plain(ManaColor::Red, trig.source); n as usize];
    vec![Effect::AddMana { player: trig.controller, mana }]
}
