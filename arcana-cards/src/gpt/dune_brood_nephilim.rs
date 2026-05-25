//! Dune-Brood Nephilim — `{B}{R}{G}{W}` 3/3 BRGW creature. "Whenever this
//! creature deals combat damage to a player, create a 1/1 colorless Sand
//! creature token for each land you control."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dune-Brood Nephilim");
    let nephilim = reg.interner_mut().intern("Nephilim");
    let _sand = reg.interner_mut().intern("Sand");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nephilim);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_create_sand_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn combat_damage_create_sand_tokens(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let land_count = script::count_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    if land_count == 0 {
        return Vec::new();
    }
    let mut effects = Vec::new();
    for _ in 0..land_count {
        let sand2 = reg.interner().lookup("Sand").expect("Sand interned");
        let mut st = SubtypeSet::default();
        st.0.insert(sand2);
        effects.push(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: sand2,
                colors: ColorSet::colorless(),
                types: TypeLine::CREATURE.into(),
                subtypes: st,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        });
    }
    effects
}
