//! Sunspine Lynx — `{2}{R}{R}` 5/4 Elemental Cat.
//!
//! Oracle:
//! * Players can't gain life.  (GAP — a static replacement/rule-altering
//!   continuous ability; no demonstrated API.)
//! * Damage can't be prevented.  (GAP — a static rule-altering ability; no
//!   demonstrated API.)
//! * When this creature enters, it deals damage to each player equal to the
//!   number of nonbasic lands that player controls.  (ETB trigger, wired.)

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sunspine Lynx");
    let elemental = reg.interner_mut().intern("Elemental");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_damage_per_nonbasic,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_damage_per_nonbasic(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Nonbasic land = LAND type, without the BASIC supertype.
    let nonbasic_land = ObjectFilter::permanent()
        .with_types(TypeLine::LAND.into())
        .without_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC));
    let effects: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| {
            let n = script::count_matching(state, &nonbasic_land, p);
            Effect::DealDamage {
                source: trig.source,
                target: DamageTarget::Player(p),
                amount: n,
            }
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
