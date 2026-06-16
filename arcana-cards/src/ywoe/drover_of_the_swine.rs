//! Drover of the Swine — `{2}{W}{B}{G}` 5/5 Human Warlock.
//! "When Drover of the Swine enters, choose one —
//!  • Conjure a card … from the Three Pigs spellbook (not modeled).
//!  • Return up to three target Boar creature cards with different
//!    names from your graveyard to the battlefield."
//!
//! GAP: modal "choose one" is only expressible on SPELL abilities, not
//! triggered abilities — the engine's modal dispatcher reads a Spell
//! stack entry's chosen modes. The Conjure mode is also unmodeled
//! (Arena-only spellbook mechanic). Best-effort: wire the ETB to the
//! single expressible mode (return up to three Boar cards); the
//! player's mode choice and the Conjure mode are dropped.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drover of the Swine");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let boar = reg.interner_mut().intern("Boar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    let boar_in_gy = ObjectFilter::creature().with_subtype_sym(boar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_return_boars,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: boar_in_gy,
                    },
                    count: TargetCount::UpTo(3),
                    controller: None,
                }],
            }),
    )
}

fn etb_return_boars(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "different names" cannot be enforced — documented fidelity gap.
    trig.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => {
                Some(Effect::ReturnFromGraveyardToBattlefield { target: *id })
            }
            _ => None,
        })
        .collect()
}
