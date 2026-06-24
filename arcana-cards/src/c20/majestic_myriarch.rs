//! Majestic Myriarch — `{4}{G}` */* Chimera (G).
//!
//! Oracle:
//! * Majestic Myriarch's power and toughness are each equal to twice the
//!   number of creatures you control — a characteristic-defining ability wired
//!   at Layer 7a via a `SelfEntersBattlefield` `self_pt_cda`. Both `*` axes
//!   resolve to `2 × (creatures you control)`. (`self_pt_from_match` counts a
//!   filter 1:1 and so cannot express the ×2 multiplier — `self_pt_cda` reads
//!   the live battlefield and doubles the count.)
//! * GAP: "At the beginning of each combat, this creature gains flying … if you
//!   control a creature with flying. The same is true for first strike, …,
//!   vigilance." — each grant must be gated on "you control a creature with
//!   <that keyword>"; expressing the per-keyword conditional grant set is
//!   outside the demonstrated trigger/effect surface, so the begin-combat
//!   ability is GAP'd whole.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Majestic Myriarch");
    let chimera = reg.interner_mut().intern("Chimera");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chimera);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // `*/*` — both axes are a CDA: twice the number of creatures you
        // control. Resolved at Layer 7a by install_cda.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
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

/// Layer 7a self-CDA: P/T each equal to twice the number of creatures you
/// control.
fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// P/T = twice the number of creatures you control.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = s
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == who && o.characteristics.types.is_creature())
        .count() as i32;
    (2 * n, 2 * n)
}
