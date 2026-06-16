//! Vihaan, Goldwaker — `{R}{W}{B}` 3/3 Legendary Dwarf Warlock.
//! Other outlaws you control have vigilance and haste. (static — GAP)
//! At the beginning of combat on your turn, you may have Treasures you
//! control become 3/3 Construct Assassin artifact creatures in addition
//! to their other types until end of turn.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vihaan, Goldwaker");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(warlock);

    // GAP: static "Other outlaws you control have vigilance and haste" — a
    // continuous static keyword-grant, not a triggered/activated ability.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: animate_treasures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn animate_treasures(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "have Treasures you control become 3/3 ... artifact creatures in addition
    // to their other types until end of turn." We animate each Treasure: add the
    // CREATURE type and set base P/T to 3/3. The "you may" choice and the
    // Construct/Assassin subtype additions are not expressible — GAP'd.
    let filter = script::subtype_filter(reg, "Treasure")
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    let mut effects = Vec::new();
    for id in ids {
        effects.push(Effect::AddType {
            target: id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        });
        effects.push(Effect::SetBasePT {
            target: id,
            power: 3,
            toughness: 3,
            duration: Duration::EndOfTurn,
        });
        // GAP: adding Construct + Assassin subtypes (no subtype-grant effect).
    }
    // GAP: "you may" — the optional choice is not modeled; effect always applies.
    effects
}
