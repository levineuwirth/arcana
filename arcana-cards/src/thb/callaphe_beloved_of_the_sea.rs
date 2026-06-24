//! Callaphe, Beloved of the Sea — `{1}{U}{U}` */3 Legendary Enchantment
//! Creature — Demigod.
//! Callaphe's power is equal to your devotion to blue.
//! Creatures and enchantments you control have "Spells your opponents cast
//! that target this permanent cost {1} more to cast."
//!
//! The `*/3` characteristic-defining P/T (power = devotion to blue,
//! toughness fixed 3 — ASYMMETRIC) is wired at Layer 7a via a
//! SelfEntersBattlefield self_pt_cda whose compute returns
//! `(devotion to blue, 3)`. Power bones use `PtValue::Star`; toughness is
//! the printed `Fixed(3)`.
//!
//! GAP (static): "Creatures and enchantments you control have 'Spells your
//! opponents cast that target this permanent cost {1} more to cast.'" — a
//! static granting a cost-increase ability to a set of permanents; not a
//! triggered/activated ability and not expressible here.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Callaphe, Beloved of the Sea");
    let demigod = reg.interner_mut().intern("Demigod");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demigod);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

// Power = your devotion to blue; toughness = printed 3.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = script::devotion(s, who, ColorSet::blue()) as i32;
    (n, 3)
}
