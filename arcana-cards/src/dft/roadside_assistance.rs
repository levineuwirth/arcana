//! Roadside Assistance — `{2}{W}` enchantment — Aura.
//! "Enchant creature or Vehicle. When this Aura enters, create a 1/1
//! colorless Pilot creature token with '...saddles Mounts and crews
//! Vehicles as though its power were 2 greater.' Enchanted permanent gets
//! +1/+1 and has lifelink."
//!
//! ETB creates the Pilot token; the attached grant is +1/+1 plus
//! lifelink. The token's saddle/crew power-boost rider text has no engine
//! representation (a plain 1/1 Pilot is emitted instead).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Roadside Assistance");
    let aura = reg.interner_mut().intern("Aura");
    let _pilot = reg.interner_mut().intern("Pilot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // NOTE: "creature or Vehicle" widened to any permanent
            .with_enchant(TargetFilter::Permanent(ObjectFilter::permanent()))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let pilot = reg
        .interner()
        .lookup("Pilot")
        .expect("Pilot interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(pilot);
    // GAP: Pilot token's saddle/crew power-boost rider text — plain 1/1 token.
    let token = TokenDefinition {
        name: pilot,
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: state
                .objects
                .get(trig.source)
                .map(|o| o.controller)
                .unwrap_or(0),
            token,
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt(
                trig.source,
                1,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Lifelink,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
